package systems.toni.hello.webserver

import javax.ws.rs.GET
import javax.ws.rs.Path
import javax.ws.rs.Produces
import javax.ws.rs.core.Context
import javax.ws.rs.core.MediaType
import javax.ws.rs.core.UriInfo

@Path("/")
class GreetingResource {

    @GET
    @Path("{path}")
    @Produces(MediaType.TEXT_PLAIN)
    fun hello(@Context ui: UriInfo) = "hello, you've hit " + ui.getPath()
}
